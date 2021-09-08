using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Utilities {
    [Serializable]
    public class Range {
        public float min;
        public float max;

        public Range(float min, float max) {
            this.min = min;
            this.max = max;
        }
    }
}
