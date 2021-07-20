using MatrixEngine.App;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.Scenes;

namespace MatrixEngineTests {
    class Program {
        static void Main(string[] args) {
            App app = new App("Tests", new Scene(new GameObject[] { 
                new GameObject(new DebugComponent()),
            }));
            
            
            app.Run();

        }
    }
}
