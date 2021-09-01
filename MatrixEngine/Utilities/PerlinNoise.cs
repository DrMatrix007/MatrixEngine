using System;
using System.Collections.Generic;
using System.Linq;


namespace MatrixEngine.Utilities {

    public class PerlinNoise {



        public readonly float[,] floats;

        public readonly int density;

        public bool isGenerated = false;
        public float step
        {
            get { return 1.0f / density; }
        }

        public readonly Random random;

        public readonly Seed seed;


        public PerlinNoise(Seed seed, int density) {
            floats = new float[density, density];
            this.density = density;
            random = new Random(seed.seed);

            this.seed = seed;

        }

        public IEnumerator<float> GenerateEnumerator() {
            isGenerated = true;
            for (int x = 0; x < density; x++) {
                for (int y = 0; y < density; y++) {
                    var v = (float)random.NextDouble();
                    floats[x, y] = v;
                    yield return v;
                }
            }
        }
        public void Generate() {
            isGenerated = true;

            for (int x = 0; x < density; x++) {
                for (int y = 0; y < density; y++) {
                    floats[x, y] = (float)random.NextDouble();
                }
            }
        }
        private float Get(float x, float y) {
            var dx = x * density;
            var dy = y * density;

            var startx = (int)Math.Clamp(Math.Floor(dx), 0, density - 1);
            var starty = (int)Math.Clamp(Math.Floor(dy), 0, density - 1);

            var endx = Math.Clamp(startx + 1, 0, density - 1);
            var endy = Math.Clamp(starty + 1, 0, density - 1);

            var val = new List<float>() {
          Lerp(floats[startx,starty],
          floats[startx,endy],dy%1),

          Lerp(floats[endx,starty],
          floats[endx,endy],dy%1),

          Lerp(floats[startx,endy],
              floats[endx,endy],dx%1),

          Lerp(floats[startx,starty],
              floats[endx,starty],dx%1)


      }.Average();
            return val;


        }

        public float this[float x, float y]
        {
            get {
                if (!isGenerated) {
                    throw new Exception("Didn't call .Generate() or .GenerateEnumerator(), you should call it.");
                }

                return Get(x, y);
            }
        }

        private float Lerp(float a, float b, float t) {
            return a * (1 - t) + b * t;
        }
    }

}
