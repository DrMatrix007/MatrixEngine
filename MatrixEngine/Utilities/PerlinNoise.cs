using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Utilities {

    public class PerlinNoise {



        public readonly float[,] floats;

        public readonly int size;

        public readonly int density;

        public bool isGenerated = false;
        public float step
        {
            get { return 1.0f / density; }
        }

        public readonly Random random;

        public readonly Seed seed;


        public PerlinNoise(Seed seed, int size, int density) {
            floats = new float[density * size, density * size];
            this.size = size;
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

            for (int x = 0; x < size * density; x += density) {
                for (int y = 0; y < size * density; y += density) {
                    floats[x, y] = (float)random.NextDouble();
                }
            }
            var devx = 0.0f;
            var devy = 0.0f;
            for (int x = 0; x < size * density; x++) {
                for (int y = 0; y < size * density; y++) {
                    //if (x % density == 0 || y % density == 0) {
                    //    continue;
                    //}
                    var startx = x / density;
                    var starty = y / density;
                    var endx = Math.Clamp(startx+1,0,(size*density-1)/density);
                    var endy = Math.Clamp(starty+1,0,(size*density-1)/density);

                    devx = ((float)(x % density)) / density;
                    devy = ((float)(y % density)) / density;

                    floats[x, y] = new List<float>() {
                        PerlinNoise.Lerp(
                            floats[startx*density,starty*density],
                            floats[startx*density,endy*density],
                            devy
                        ),
                        PerlinNoise.Lerp(
                            floats[endx*density,starty*density],
                            floats[endx*density,endy*density],
                            devy
                        ),
                        PerlinNoise.Lerp(
                            floats[startx*density,endy*density],
                            floats[endx*density,endy*density],
                            devx
                        ),
                        PerlinNoise.Lerp(
                            floats[startx*density,starty*density],
                            floats[endx*density,starty*density],
                            devx
                        ),


                    }.Average();
                    //Console.WriteLine(floats[startx * density, starty * density]==floats[endx * density, starty * density]);

                }
            }

        }
        private float Get(int x, int y) {

            return floats[x, y];


        }

        public float this[int x, int y]
        {
            get {
                if (!isGenerated) {
                    throw new Exception("Didn't call .Generate() or .GenerateEnumerator(), you should call it.");
                }

                return Get(x, y);
            }
        }

        //private float Lerp(float a, float b, float t) {
        //    return a * (1 - t) + b * t;
        //}
        static float Cubic(float f) {
            return (float)(-Math.Cos(f * Math.PI) + 1) / 2;
        }

        static float Lerp(float firstFloat, float secondFloat, float by) {
            return (firstFloat * (1 - Cubic(by))) + secondFloat * Cubic(by);
        }
    }

}
