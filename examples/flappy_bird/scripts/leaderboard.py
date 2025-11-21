"""
Leaderboard System for Flappy Bird
Manages high scores with username tracking and persistent storage.
"""

import json
import os
from datetime import datetime

class LeaderboardEntry:
    """A single leaderboard entry."""
    
    def __init__(self, username, score, timestamp=None):
        """
        Initialize a leaderboard entry.
        
        Args:
            username: Player username
            score: Player score
            timestamp: ISO format timestamp (auto-generated if None)
        """
        self.username = username
        self.score = score
        self.timestamp = timestamp or datetime.now().isoformat()
    
    def to_dict(self):
        """Convert entry to dictionary."""
        return {
            'username': self.username,
            'score': self.score,
            'timestamp': self.timestamp
        }
    
    @staticmethod
    def from_dict(data):
        """Create entry from dictionary."""
        return LeaderboardEntry(
            data['username'],
            data['score'],
            data.get('timestamp')
        )
    
    def __repr__(self):
        return f"LeaderboardEntry({self.username}, {self.score})"


class Leaderboard:
    """Manages a leaderboard with persistent storage."""
    
    def __init__(self, save_file="flappy_bird_leaderboard.json", max_entries=10):
        """
        Initialize the leaderboard.
        
        Args:
            save_file: Path to save file (relative to flappy_bird directory)
            max_entries: Maximum number of entries to keep
        """
        # Get path relative to the flappy_bird directory (parent of scripts)
        flappy_bird_dir = os.path.dirname(os.path.dirname(__file__))
        self.save_path = os.path.join(flappy_bird_dir, save_file)
        self.max_entries = max_entries
        self.entries = []
        self.load()
    
    def add_entry(self, username, score):
        """
        Add a new entry to the leaderboard, only replacing existing entry if new score is higher.
        
        Args:
            username: Player username
            score: Player score
        
        Returns:
            int: Rank of the new entry (1-indexed), or None if not in top entries
        """
        # Check if username already exists
        existing_entry = None
        for e in self.entries:
            if e.username == username:
                existing_entry = e
                break
        
        # If username exists and has a higher score, don't replace
        if existing_entry and existing_entry.score > score:
            # Return the rank of their existing better score
            rank = self.entries.index(existing_entry) + 1
            return rank if rank <= self.max_entries else None
        
        # Remove existing entry with same username (if any) since new score is better
        if existing_entry:
            self.entries.remove(existing_entry)
        
        # Add new entry
        entry = LeaderboardEntry(username, score)
        self.entries.append(entry)
        
        # Sort by score (descending)
        self.entries.sort(key=lambda e: e.score, reverse=True)
        
        # Find rank of new entry
        rank = None
        for i, e in enumerate(self.entries):
            if e is entry:
                rank = i + 1
                break
        
        # Keep only top entries
        self.entries = self.entries[:self.max_entries]
        
        # Save to disk
        self.save()
        
        # Return rank if entry is still in the list
        return rank if rank and rank <= self.max_entries else None
    
    def get_top_entries(self, count=None):
        """
        Get the top leaderboard entries.
        
        Args:
            count: Number of entries to return (None for all)
        
        Returns:
            list: List of LeaderboardEntry objects
        """
        if count is None:
            return self.entries.copy()
        return self.entries[:count]
    
    def get_rank(self, score):
        """
        Get the rank a score would have on the leaderboard.
        
        Args:
            score: Score to check
        
        Returns:
            int: Rank (1-indexed), or None if not in top entries
        """
        for i, entry in enumerate(self.entries):
            if score > entry.score:
                return i + 1
        
        # Check if it would be at the end
        if len(self.entries) < self.max_entries:
            return len(self.entries) + 1
        
        return None
    
    def is_high_score(self, score):
        """
        Check if a score qualifies for the leaderboard.
        
        Args:
            score: Score to check
        
        Returns:
            bool: True if score is in top max_entries
        """
        if len(self.entries) < self.max_entries:
            return True
        return score > self.entries[-1].score
    
    def get_high_score(self):
        """
        Get the highest score.
        
        Returns:
            int: Highest score, or 0 if no entries
        """
        if self.entries:
            return self.entries[0].score
        return 0
    
    def clear(self):
        """Clear all leaderboard entries."""
        self.entries.clear()
        self.save()
    
    def save(self):
        """Save leaderboard to disk."""
        try:
            data = {
                'entries': [entry.to_dict() for entry in self.entries],
                'max_entries': self.max_entries,
                'last_updated': datetime.now().isoformat()
            }
            
            with open(self.save_path, 'w') as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            print(f"Warning: Failed to save leaderboard: {e}")
    
    def load(self):
        """Load leaderboard from disk."""
        try:
            if os.path.exists(self.save_path):
                with open(self.save_path, 'r') as f:
                    data = json.load(f)
                
                self.entries = [LeaderboardEntry.from_dict(e) for e in data.get('entries', [])]
                self.max_entries = data.get('max_entries', self.max_entries)
                
                # Sort by score (descending)
                self.entries.sort(key=lambda e: e.score, reverse=True)
            else:
                # Create empty leaderboard file
                self.entries = []
                self.save()
        except Exception as e:
            print(f"Warning: Failed to load leaderboard: {e}")
            self.entries = []
    
    def __str__(self):
        """String representation of leaderboard."""
        if not self.entries:
            return "Leaderboard is empty"
        
        lines = ["=== LEADERBOARD ==="]
        for i, entry in enumerate(self.entries):
            lines.append(f"{i+1}. {entry.username}: {entry.score}")
        return "\n".join(lines)

