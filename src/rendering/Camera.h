//
// Created by Aram Aprahamian on 11/22/25.
//

#ifndef CAMERA_H
#define CAMERA_H

namespace pyg {

class Camera {
    public:
        Camera();
        virtual ~Camera();

        virtual void setView(View* view);
        virtual View* getView() const;
        virtual void setProjection(Projection* projection);
        virtual Projection* getProjection() const;
        virtual void setRenderTarget(const RenderTarget& renderTarget);
        virtual RenderTarget getRenderTarget() const;
        virtual void setRenderTexture(const RenderTexture& renderTexture);
        virtual RenderTexture getRenderTexture() const;
        virtual void setRenderWindow(const RenderWindow& renderWindow);
        virtual RenderWindow getRenderWindow() const;
        virtual void setRenderView(const View& renderView);
        virtual View getRenderView() const;
        virtual void setRenderSize(const Vector2u& renderSize);
        virtual Vector2u getRenderSize() const;
        virtual void setRenderPosition(const Vector2i& renderPosition);
        virtual Vector2i getRenderPosition() const;
        virtual void setRenderClearColor(const Color& renderClearColor);
        virtual Color getRenderClearColor() const;
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

    private:
        View* _view;
        Projection* _projection;
        RenderTarget _renderTarget;
        RenderTexture _renderTexture;
        RenderWindow _renderWindow;
        View _renderView;
        Vector2u _renderSize;
        Vector2i _renderPosition;
        Color _renderClearColor;
        float _renderClearDepth;
        int _renderClearStencil;
        RenderTarget _renderClearTarget;
        bool _renderClearAll;
        bool _renderFlip;
        bool _renderDisplay;

};

} // pyg

#endif //CAMERA_H
