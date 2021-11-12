using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utils
{
    public class MatrixRandom
    {
        private Random random;

        public MatrixRandom(int seed)
        {
            random = new Random(seed);
        }
        public bool RandomBool()
        {
            return random.Next(2) == 0;
        }
        public float RandomFloat()
        {
            return (float)random.NextDouble();
        }
        public double RandomDouble()
        {
            return random.NextDouble();
        }


    }
}
