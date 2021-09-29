using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utilities {

    [Serializable]
    public struct Range {
        public static readonly Range ZeroToOne = new Range(0, 1);
        public float min;
        public float max;

        public Range(float min, float max) {
            this.min = min;
            this.max = max;
        }
    }
}