using MatrixEngine.Framework;
using SFML.Graphics;
using SFML.System;
using SFML.Window;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Content;

namespace MatrixEngine.Testing {
    public class TestingWindow {
        public (int width, int height) gridSize;

        public List<(object obj,string source)> objects;

        RenderWindow window;

        private int toAddSeed = new Random().Next(0,1000);
        private Vector2f offset = new Vector2f(0,0);

        public TestingWindow((int width,int height) gridSize) {

            var screen_size = VideoMode.DesktopMode;


            window = new RenderWindow(new VideoMode(screen_size.Width-10,screen_size.Height/5),"Testing",Styles.Close);
            this.gridSize = gridSize;
            objects = new List<(object obj,string source)>();


            window.Position = new Vector2i(0, (int)(screen_size.Height * ((float)4/5))-100);

            window.Closed += (sender,args) => {
                window.Close();
            };

        }
        public void Add<T>(T @object) where T:class {
            objects.Add((@object, Environment.StackTrace));
        }

        public void Update() {
            window.Clear(Color.Black);
            
            window.DispatchEvents();

            window.SetView(new View(((Vector2f)window.Size)/2+offset, (Vector2f)window.Size));




            Draw();

            

            window.Display();
        }

        void Draw() {
            var size = window.Size;

            var cellxs = (uint)size.X / (uint)gridSize.width;
            var cellys = (uint)size.Y / (uint)gridSize.height;

            var l = objects.ToList();
            var finalpos = l.Count - 1;

            var clampValue = 0;

            for (int i = 0; i < l.Count; i++) {
                
                var obj = l[i];
                var x = i % (gridSize.width+1 );
                var y = i/ (gridSize.height + 1);
                x*=(int)cellxs;
                y*=(int)cellys;


                if (i == finalpos) {
                    clampValue = y;
                }

                var t = new RenderTexture(cellxs,cellys);

                var r = new Random(i+toAddSeed);
                offset.X = Math.Clamp(offset.X,0 , clampValue);
                
                t.Clear(new Color((byte)r.Next(254),(byte)r.Next(254),(byte)r.Next(254)));
                var te =(obj.obj).ToString();
                t.Draw(new Text(te,FontManager.CascadiaCode,20));

                var s = new Sprite(t.Texture);
                s.Position = new Vector2f(x, y+cellys);
                s.Scale = new Vector2f(1, -1);
                window.Draw(s);

                s.Dispose();
                t.Dispose();
            }

        }



    }
}
