using MatrixEngine.Framework;

namespace MatrixEngine.Scenes.Plugins {
    public abstract class Plugin {

        public App App => scene?.app;

        public InputHandler InputHandler => App?.InputHandler;



        public Scene scene
        {
            get;
            private set;
        }

        internal void SetupScene(Scene s) {
            this.scene = s;
        }

        internal bool HasStarted = false;

        public abstract void Start();

        public abstract void Update();

        public abstract void LateUpdate();


    }
}
