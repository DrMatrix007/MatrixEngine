using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utils
{
    [Serializable]
    public struct MatrixRange
    {
        public static readonly MatrixRange ZeroToOne = new MatrixRange(0, 1);
        public float min;
        public float max;

        public MatrixRange(float min, float max)
        {
            this.min = min;
            this.max = max;
        }
        public MatrixRange(float max)
        {
            this.min = 0;
            this.max = max;
        }
    }
}
