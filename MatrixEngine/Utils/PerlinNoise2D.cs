using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utils
{
    public class PerlinNoise2D
    {
        public readonly int density = 16;

        private MatrixRandom random;

        public readonly int size;

        public readonly int fullSize;

        public float[,] floats
        {
            private set;
            get;
        }

        private MatrixRange range;

        public PerlinNoise2D(int s, int density, MatrixRange range, MatrixRandom rand)
        {
            this.density = density;
            random = rand;
            this.range = range;
            this.fullSize = this.density * s;
            size = s % this.density == 0 ? s : s + this.density - s % this.density;
            floats = new float[size * this.density, size * this.density];
        }

        public PerlinNoise2D(int s, int density, MatrixRange range, int seed): this(s,density,range,new MatrixRandom(seed))
        {

        }

        private static float Cubic(float f)
        {
            //return f;
            return (-2 * f +3)*f* f;

            //return (-MathF.Cos(f * MathF.PI) + 1) / 2;
        }

        private static float Lerp(float firstFloat, float secondFloat, float by)
        {
            var a = Cubic(by);
            return (firstFloat * (1 - a) + secondFloat * a);
        }


        public void Generate()
        {
            var randoms = new float[size, size];
            for (int x = 0; x < size; x++)
            {
                for (int y = 0; y < size; y++)
                {
                    randoms[x, y] = (random.RandomFloat() * (range.max - range.min) + range.min);
                }
            }
            for (int x = 0; x < size - 1; x++)
            {
                for (int y = 0; y < size - 1; y++)
                {
                    var upleft = randoms[x, y];
                    var downleft = randoms[x, y + 1];
                    var upright = randoms[x + 1, y];
                    var downright = randoms[x + 1, y + 1];

                    for (int dx = 0; dx <= density; dx += 1)
                    {
                        for (int dy = 0; dy <= density; dy += 1)
                        {
                            var topl = Lerp(upleft, upright, (float)dx/density);
                            var downl = Lerp(downleft, downright, (float)dx/density);
                            floats[(int)(x * density + dx ), (int)(y * density + dy )] = (Lerp(topl, downl, (float)dy/density));
                            //Console.WriteLine(floats[(int)(x + dx * RandomGenerationSize), (int)(y + dy * RandomGenerationSize)]);
                        }
                    }
                }
            }

            //Parallel.For(0, size - 1, (x) => {
            //    Parallel.For(0, size - 1, (y) => {
            //        var upleft = randoms[x, y];
            //        var downleft = randoms[x, y + 1];
            //        var upright = randoms[x + 1, y];
            //        var downright = randoms[x + 1, y + 1];

            //        var stepx = (float)1 / randomGenerationSize;

            //        var stepy = (float)1 / randomGenerationSize;

            //        for (float dx = 0; dx <= 1; dx += stepx)
            //        {
            //            for (float dy = 0; dy <= 1; dy += stepy)
            //            {
            //                var topl = Lerp(upleft, upright, dx);
            //                var downl = Lerp(downleft, downright, dx);
            //                floats[(int)(x * randomGenerationSize + dx * randomGenerationSize), (int)(y * randomGenerationSize + dy * randomGenerationSize)] = (Lerp(topl, downl, dy));
            //                //Console.WriteLine(floats[(int)(x + dx * RandomGenerationSize), (int)(y + dy * RandomGenerationSize)]);
            //            }
            //        }
            //    });
            //});

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
