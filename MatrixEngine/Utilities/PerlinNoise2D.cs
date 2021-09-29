using SFML.System;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace MatrixEngine.Utilities {

    public class PerlinNoise2D {
        public readonly int randomGenerationSize = 16;

        private Random random = new Random();

        public readonly int size;

        public readonly int fullSize;

        public float[,] floats
        {
            private set;
            get;
        }

        private Range range;

        private static float Cubic(float f) {
            return (-MathF.Cos(f * MathF.PI) + 1) / 2;
        }

        private static float Lerp(float firstFloat, float secondFloat, float by) {
            var a = Cubic(by);
            return (firstFloat * (1 - a) + secondFloat * a);
        }

        public PerlinNoise2D(int s, int density, Range range) {
            randomGenerationSize = density;
            this.range = range;
            this.fullSize = randomGenerationSize * s;
            size = s % randomGenerationSize == 0 ? s : s + randomGenerationSize - s % randomGenerationSize;
            floats = new float[size * randomGenerationSize, size * randomGenerationSize];
        }

        public void Generate() {
            var randoms = new float[size, size];
            var c = new Clock();
            for (int x = 0; x < size; x++) {
                for (int y = 0; y < size; y++) {
                    randoms[x, y] = ((float)random.NextDouble() * (range.max - range.min) + range.min);
                }
            }
            c.ElapsedTime.AsSeconds().Log();
            c.Restart();
            //for (int x = 0; x < size - 1; x++) {
            //    for (int y = 0; y < size - 1; y++) {
            //        var upleft = randoms[x, y];
            //        var downleft = randoms[x, y + 1];
            //        var upright = randoms[x + 1, y];
            //        var downright = randoms[x + 1, y + 1];

            //        for (float dx = 0; dx <= 1; dx += (float)1 / randomGenerationSize) {
            //            for (float dy = 0; dy <= 1; dy += (float)1 / randomGenerationSize) {
            //                var topl = Lerp(upleft, upright, dx);
            //                var downl = Lerp(downleft, downright, dx);
            //                floats[(int)(x * randomGenerationSize + dx * randomGenerationSize), (int)(y * randomGenerationSize + dy * randomGenerationSize)] = (Lerp(topl, downl, dy));
            //                //Console.WriteLine(floats[(int)(x + dx * RandomGenerationSize), (int)(y + dy * RandomGenerationSize)]);
            //            }
            //        }
            //    }
            //}

            Parallel.For(0, size - 1, (x) => {
                Parallel.For(0, size - 1, (y) => {
                    var upleft = randoms[x, y];
                    var downleft = randoms[x, y + 1];
                    var upright = randoms[x + 1, y];
                    var downright = randoms[x + 1, y + 1];

                    var stepx = (float)1 / randomGenerationSize;

                    var stepy = (float)1 / randomGenerationSize;

                    for (float dx = 0; dx <= 1; dx += stepx) {
                        for (float dy = 0; dy <= 1; dy += stepy) {
                            var topl = Lerp(upleft, upright, dx);
                            var downl = Lerp(downleft, downright, dx);
                            floats[(int)(x * randomGenerationSize + dx * randomGenerationSize), (int)(y * randomGenerationSize + dy * randomGenerationSize)] = (Lerp(topl, downl, dy));
                            //Console.WriteLine(floats[(int)(x + dx * RandomGenerationSize), (int)(y + dy * RandomGenerationSize)]);
                        }
                    }
                });
            });

            c.ElapsedTime.AsSeconds().Log();
        }

        //public IEnumerator<float> GetEnumerator() {
        //    if (floats == null) {
        //        throw new Exception("\"Generate()\" have't been called yet");
        //    }
        //    foreach (var f in floats) {
        //        yield return f;
        //    }
        //}
    }
}