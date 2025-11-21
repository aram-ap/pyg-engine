"""
Serialization system for GameObjects and components.
Handles conversion to/from JSON format with support for custom types.
"""

import json
import inspect
from typing import Any, Dict, List, Optional, Union
from ..core.gameobject import GameObject
from ..components.component import Component
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from ..utilities.object_types import Tag, BasicShape


class SerializationError(Exception):
    """Exception raised during serialization/deserialization."""
    pass


def serialize_game_object(obj: GameObject) -> Dict[str, Any]:
    """
    Serialize a GameObject to a dictionary.
    
    Args:
        obj: GameObject to serialize
        
    Returns:
        Dictionary representation of the GameObject
    """
    if not isinstance(obj, GameObject):
        raise SerializationError(f"Expected GameObject, got {type(obj)}")
    
    data = {
        'type': 'GameObject',
        'name': obj.name,
        'id': obj.id,
        'enabled': obj.enabled,
        'position': serialize_value(obj.position),
        'size': serialize_value(obj.size),
        'rotation': obj.rotation,
        'color': serialize_value(obj.color),
        'tag': serialize_value(obj.tag),
        'basicShape': serialize_value(obj.basicShape),
        'show_rotation_line': obj.show_rotation_line,
        'components': []
    }
    
    # Serialize all components
    for component_class, component in obj.components.items():
        try:
            component_data = serialize_component(component)
            component_data['_component_type'] = component_class.__name__
            component_data['_component_module'] = component_class.__module__
            data['components'].append(component_data)
        except Exception as e:
            # Skip components that can't be serialized
            print(f"Warning: Could not serialize component {component_class.__name__}: {e}")
            continue
    
    return data


def serialize_component(component: Component) -> Dict[str, Any]:
    """
    Serialize a component to a dictionary.
    
    Args:
        component: Component to serialize
        
    Returns:
        Dictionary representation of the component
    """
    if not isinstance(component, Component):
        raise SerializationError(f"Expected Component, got {type(component)}")
    
    data = {
        'enabled': component.enabled,
    }
    
    # Get all attributes of the component
    for attr_name in dir(component):
        # Skip private attributes, methods, and special attributes
        if attr_name.startswith('_'):
            continue
        
        try:
            attr_value = getattr(component, attr_name)
            # Skip methods and callables
            if callable(attr_value):
                continue
            
            # Skip the game_object reference (circular)
            if attr_name == 'game_object':
                continue
            
            # Try to serialize the value
            try:
                data[attr_name] = serialize_value(attr_value)
            except Exception as e:
                # Skip attributes that can't be serialized
                print(f"Warning: Could not serialize attribute {attr_name}: {e}")
                continue
        except Exception:
            # Skip attributes that can't be accessed
            continue
    
    return data


def serialize_value(value: Any) -> Any:
    """
    Serialize a value to a JSON-serializable format.
    
    Args:
        value: Value to serialize
        
    Returns:
        JSON-serializable representation
    """
    # Handle None
    if value is None:
        return None
    
    # Handle Vector2
    if isinstance(value, Vector2):
        return {'_type': 'Vector2', 'x': value.x, 'y': value.y}
    
    # Handle Color
    if isinstance(value, Color):
        return {'_type': 'Color', 'r': value.r, 'g': value.g, 'b': value.b, 'a': value.a}
    
    # Handle Tag enum
    if isinstance(value, Tag):
        return {'_type': 'Tag', 'value': value.value}
    
    # Handle BasicShape enum
    if isinstance(value, BasicShape):
        return {'_type': 'BasicShape', 'value': value.value}
    
    # Handle lists
    if isinstance(value, (list, tuple)):
        return [serialize_value(item) for item in value]
    
    # Handle dictionaries
    if isinstance(value, dict):
        return {key: serialize_value(val) for key, val in value.items()}
    
    # Handle basic types
    if isinstance(value, (str, int, float, bool)):
        return value
    
    # Try to convert to string for unknown types
    try:
        return str(value)
    except Exception:
        raise SerializationError(f"Cannot serialize value of type {type(value)}")


def deserialize_value(value: Any) -> Any:
    """
    Deserialize a value from JSON format.
    
    Args:
        value: JSON value to deserialize
        
    Returns:
        Deserialized Python object
    """
    # Handle None
    if value is None:
        return None
    
    # Handle type markers
    if isinstance(value, dict) and '_type' in value:
        type_name = value['_type']
        
        if type_name == 'Vector2':
            return Vector2(value['x'], value['y'])
        elif type_name == 'Color':
            return Color(value['r'], value['g'], value['b'], value['a'])
        elif type_name == 'Tag':
            return Tag(value['value'])
        elif type_name == 'BasicShape':
            return BasicShape(value['value'])
    
    # Handle lists
    if isinstance(value, list):
        return [deserialize_value(item) for item in value]
    
    # Handle dictionaries (recursive)
    if isinstance(value, dict):
        return {key: deserialize_value(val) for key, val in value.items()}
    
    # Return as-is for basic types
    return value


def deserialize_game_object(data: Dict[str, Any], engine=None) -> GameObject:
    """
    Deserialize a GameObject from a dictionary.
    
    Args:
        data: Dictionary representation of GameObject
        engine: Optional engine reference (for component initialization)
        
    Returns:
        Deserialized GameObject
    """
    if data.get('type') != 'GameObject':
        raise SerializationError("Invalid GameObject data")
    
    # Create GameObject with basic properties
    obj = GameObject(
        name=data['name'],
        id=data.get('id'),
        enabled=data.get('enabled', True),
        position=deserialize_value(data.get('position', {'_type': 'Vector2', 'x': 0, 'y': 0})),
        size=deserialize_value(data.get('size', {'_type': 'Vector2', 'x': 1, 'y': 1})),
        rotation=data.get('rotation', 0.0),
        color=deserialize_value(data.get('color')),
        tag=deserialize_value(data.get('tag')),
        basicShape=deserialize_value(data.get('basicShape')),
        show_rotation_line=data.get('show_rotation_line', False)
    )
    
    # Deserialize components (if engine is provided)
    if engine and 'components' in data:
        for component_data in data['components']:
            try:
                component_type_name = component_data.get('_component_type')
                component_module = component_data.get('_component_module')
                
                if component_type_name and component_module:
                    # Try to import and instantiate component
                    try:
                        module = __import__(component_module, fromlist=[component_type_name])
                        component_class = getattr(module, component_type_name)
                        
                        # Create component
                        component = obj.add_component(component_class)
                        
                        # Apply properties
                        for key, value in component_data.items():
                            if key.startswith('_'):
                                continue
                            try:
                                deserialized_value = deserialize_value(value)
                                if hasattr(component, key):
                                    setattr(component, key, deserialized_value)
                            except Exception as e:
                                print(f"Warning: Could not set {key} on {component_type_name}: {e}")
                    except Exception as e:
                        print(f"Warning: Could not deserialize component {component_type_name}: {e}")
            except Exception as e:
                print(f"Warning: Error deserializing component: {e}")
                continue
    
    return obj


def serialize_scene(engine) -> Dict[str, Any]:
    """
    Serialize the entire scene (all GameObjects).
    
    Args:
        engine: Engine instance
        
    Returns:
        Dictionary containing scene data
    """
    game_objects = engine.getGameObjects()
    
    return {
        'version': '1.0',
        'game_objects': [serialize_game_object(obj) for obj in game_objects]
    }


def deserialize_scene(data: Dict[str, Any], engine) -> List[GameObject]:
    """
    Deserialize a scene and add GameObjects to engine.
    
    Args:
        data: Scene data dictionary
        engine: Engine instance to add objects to
        
    Returns:
        List of deserialized GameObjects
    """
    if 'game_objects' not in data:
        raise SerializationError("Invalid scene data")
    
    objects = []
    for obj_data in data['game_objects']:
        try:
            obj = deserialize_game_object(obj_data, engine)
            engine.addGameObject(obj)
            objects.append(obj)
        except Exception as e:
            print(f"Warning: Could not deserialize GameObject: {e}")
            continue
    
    return objects


def to_json(data: Dict[str, Any], indent: int = 2) -> str:
    """Convert serialized data to JSON string."""
    return json.dumps(data, indent=indent, default=str)


def from_json(json_str: str) -> Dict[str, Any]:
    """Parse JSON string to dictionary."""
    return json.loads(json_str)

