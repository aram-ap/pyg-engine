//
// Created by Aram Aprahamian on 11/22/25.
//

#ifndef RENDERER_H
#define RENDERER_H

namespace pyg {

class Renderer {

    public:
        Renderer();
        virtual ~Renderer();

        virtual void render();
        virtual void clear();
        virtual void setView(View* view);
        virtual View* getView() const;
        virtual void setProjection(Projection* projection);
        virtual Projection* getProjection() const;

        virtual void setRenderTexture(const sf::RenderTexture& renderTexture);
        virtual sf::RenderTexture getRenderTexture() const;

        virtual void setRenderWindow(const sf::RenderWindow& renderWindow);
        virtual sf::RenderWindow getRenderWindow() const;

        virtual void setRenderView(const sf::View& renderView);
        virtual sf::View getRenderView() const;

        virtual void setRenderSize(const sf::Vector2u& renderSize);
        virtual sf::Vector2u getRenderSize() const;

        virtual void setRenderPosition(const sf::Vector2i& renderPosition);
        virtual sf::Vector2i getRenderPosition() const;

        virtual void setRenderClearColor(const sf::Color& renderClearColor);
        virtual sf::Color getRenderClearColor() const;

        virtual void setRenderClearDepth(float renderClearDepth);
        virtual float getRenderClearDepth() const;

        virtual void setRenderClearStencil(int renderClearStencil);
        virtual int getRenderClearStencil() const;

        virtual void setRenderClearTarget(const RenderTarget& renderClearTarget);
        virtual RenderTarget getRenderClearTarget() const;

        virtual void setRenderClearAll(bool renderClearAll);
        virtual bool getRenderClearAll() const;

        virtual void setRenderFlip(bool renderFlip);
        virtual bool getRenderFlip() const;

        virtual void setRenderDisplay(bool renderDisplay);
        virtual bool getRenderDisplay() const;

        virtual void setRenderTarget(const RenderTarget& renderTarget);
        virtual RenderTarget getRenderTarget() const;

        virtual void setRenderTargetName(const std::string& renderTargetName);
        virtual std::string getRenderTargetName() const;

        virtual void setRenderTextureName(const std::string& renderTextureName);
        virtual std::string getRenderTextureName() const;

        virtual void setRenderWindowName(const std::string& renderWindowName);
        virtual std::string getRenderWindowName() const;

        virtual void setRenderViewName(const std::string& renderViewName);
        virtual std::string getRenderViewName() const;

        virtual void setRenderTextureEnabled(bool renderTextureEnabled);
        virtual bool getRenderTextureEnabled() const;

        virtual void setRenderWindowEnabled(bool renderWindowEnabled);
        virtual bool getRenderWindowEnabled() const;

        virtual void setRenderViewEnabled(bool renderViewEnabled);
        virtual bool getRenderViewEnabled() const;

        virtual void setRenderProjectionEnabled(bool renderProjectionEnabled);
        virtual bool getRenderProjectionEnabled() const;

        virtual void setRenderSizeEnabled(bool renderSizeEnabled);
        virtual bool getRenderSizeEnabled() const;

        virtual void setRenderPositionEnabled(bool renderPositionEnabled);
        virtual bool getRenderPositionEnabled() const;

        virtual void setRenderClearColorEnabled(bool renderClearColorEnabled);
        virtual bool getRenderClearColorEnabled() const;

        virtual void setRenderClearDepthEnabled(bool renderClearDepthEnabled);
        virtual bool getRenderClearDepthEnabled() const;

        virtual void setRenderClearStencilEnabled(bool renderClearStencilEnabled);
        virtual bool getRenderClearStencilEnabled() const;

        virtual void setRenderClearTargetEnabled(bool renderClearTargetEnabled);
        virtual bool getRenderClearTargetEnabled() const;

        virtual void setRenderClearAllEnabled(bool renderClearAllEnabled);
        virtual bool getRenderClearAllEnabled() const;

        virtual void setRenderFlipEnabled(bool renderFlipEnabled);
        virtual bool getRenderFlipEnabled() const;

        virtual void setRenderDisplayEnabled(bool renderDisplayEnabled);
        virtual bool getRenderDisplayEnabled() const;

        virtual void setRenderTargetEnabled(bool renderTargetEnabled);
        virtual bool getRenderTargetEnabled() const;

        virtual void setRenderTarget(const std::string& renderTargetName);
        virtual std::string getRenderTarget() const;
        virtual void setRenderTexture(const std::string& renderTextureName);
        virtual std::string getRenderTexture() const;
        virtual void setRenderWindow(const std::string& renderWindowName);
        virtual std::string getRenderWindow() const;
        virtual void setRenderView(const std::string& renderViewName);
        virtual std::string getRenderView() const;

        virtual void setRenderTarget(const std::string& renderTargetName);
        virtual std::string getRenderTarget() const;

        virtual void setRenderTexture(const std::string& renderTextureName);
        virtual std::string getRenderTexture() const;

        virtual void setRenderWindow(const std::string& renderWindowName);
        virtual std::string getRenderWindow() const;

        virtual void setRenderView(const std::string& renderViewName);
        virtual std::string getRenderView() const;

        virtual void setRenderTarget(const std::string& renderTargetName);
        virtual std::string getRenderTarget() const;

        virtual void setRenderTexture(const std::string& renderTextureName);
        virtual std::string getRenderTexture() const;

        virtual void setRenderWindow(const std::string& renderWindowName);
        virtual std::string getRenderWindow() const;

        virtual void setRenderView(const std::string& renderViewName);
        virtual std::string getRenderView() const;


        virtual void setRenderTarget(const std::string& renderTargetName);
        virtual std::string getRenderView() const;
        virtual std::string getRenderTexture() const;
        virtual std::string getRenderWindow() const;
        virtual sf::View getRenderView() const;
        virtual sf::Vector2u getRenderSize() const;
        virtual sf::Vector2i getRenderPosition() const;
        virtual sf::Color getRenderClearColor() const;
        virtual float getRenderClearDepth() const;
        virtual int getRenderClearStencil() const;
        virtual RenderTarget getRenderClearTarget() const;
        virtual bool getRenderClearAll() const;
        virtual bool getRenderFlip() const;
        virtual bool getRenderDisplay() const;
        virtual bool getRenderTargetEnabled() const;
        virtual bool getRenderTextureEnabled() const;
        virtual bool getRenderWindowEnabled() const;
        virtual bool getRenderViewEnabled() const;
        virtual bool getRenderProjectionEnabled() const;
        virtual bool getRenderSizeEnabled() const;
        virtual bool getRenderPositionEnabled() const;
        virtual bool getRenderClearColorEnabled() const;
        virtual bool getRenderClearDepthEnabled() const;
        virtual bool getRenderClearStencilEnabled() const;
        virtual bool getRenderClearTargetEnabled() const;
        virtual bool getRenderClearAllEnabled() const;
        virtual bool getRenderFlipEnabled() const;
        virtual bool getRenderDisplayEnabled() const;
        virtual bool getRenderTargetEnabled() const;
        virtual bool getRenderTextureEnabled() const;
        virtual bool getRenderWindowEnabled() const;
        virtual bool getRenderViewEnabled() const;



    private:
        View* _view;
        Projection* _projection;
        sf::RenderTexture _renderTexture;
        sf::RenderWindow _renderWindow;
        sf::View _renderView;
        sf::Vector2u _renderSize;
        sf::Vector2i _renderPosition;
        bool _renderTextureEnabled;
        bool _renderWindowEnabled;
        bool _renderViewEnabled;
        bool _renderProjectionEnabled;
        bool _renderSizeEnabled;
        bool _renderPositionEnabled;
        bool _renderClearColorEnabled;
        sf::Color _renderClearColor;
        bool _renderClearDepthEnabled;
        float _renderClearDepth;
        bool _renderClearStencilEnabled;
        int _renderClearStencil;
        bool _renderClearTargetEnabled;
        RenderTarget _renderClearTarget;
        bool _renderClearAllEnabled;
        bool _renderFlipEnabled;
        bool _renderDisplayEnabled;
        bool _renderTargetEnabled;
        RenderTarget _renderTarget;
        std::string _renderTargetName;
        std::string _renderTextureName;
        std::string _renderWindowName;
        std::string _renderViewName;


};

} // pyg

#endif //RENDERER_H
