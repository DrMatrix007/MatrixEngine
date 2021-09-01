using System.ComponentModel;
using MatrixEngine.Framework;

namespace MatrixEngine.Renderers {
    public abstract class Renderer {
        public App app { private set; get; }

        public Renderer(App app) {
            this.app = app;
        }
        
        public abstract void Render();

    }
}