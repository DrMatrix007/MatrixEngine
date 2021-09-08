using System;
using System.Collections;
using System.Collections.Generic;
namespace MatrixEngine.Utilities {

    public class PerlinNoise1D : IEnumerable<float> {
        private readonly int RandomGenerationSize = 16;

        Random random = new Random();

        public int size;


        private List<float> floats;

        private Range range;

        public int fullSize { get => size * RandomGenerationSize; }

        static float Cubic(float f) {
            return (float)(-Math.Cos(f * Math.PI) + 1) / 2;
        }

        static float Lerp(float firstFloat, float secondFloat, float by) {
            return firstFloat * (1 - Cubic(by)) + secondFloat * Cubic(by);
        }

        public PerlinNoise1D(int s,int randomGenSize, Range range) {
            this.range = range;
            RandomGenerationSize = randomGenSize;
            size = s % RandomGenerationSize == 0 ? s : s + RandomGenerationSize - s % RandomGenerationSize;

        }

        public void Generate() {
            floats = new List<float>();
            List<float> randoms = new List<float>();

            for (int i = 0; i < size + 1; i++) {
                randoms.Add((float)random.NextDouble() * (range.max - range.min) + range.min);

            }

            for (int i = 0; i < randoms.Count - 1; i++) {
                for (float j = 0; j <= 1; j += (float)1 / RandomGenerationSize) {
                    floats.Add(Lerp(randoms[i], randoms[i + 1], j));

                }
            }
        }


        public IEnumerator<float> GetEnumerator() {
            if (floats == null) {
                throw new Exception("\"Generate()\" have't been called yet");
            }
            foreach (var f in floats) {
                yield return f;
            }
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }

        public float this[int i]
        {
            get {
                if (floats == null) {
                    throw new Exception("\"Generate()\" have't been called yet");
                }
                return floats[i];
            }
        }


    }


}

