using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utilities {
    public class Seed {
        public static Random random = new Random();

        public readonly int seed;

        public Seed() {
            seed = random.Next(int.MinValue, int.MaxValue);

        }
        public Seed(int seed) {
            this.seed = seed;
        }

    }
}
