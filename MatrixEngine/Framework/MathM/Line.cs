using System;
using SFML.System;

namespace MatrixEngine.Framework.MathM {
    public struct Line {
        public readonly float a;
        public readonly float b;
        public readonly float c;

        public readonly Vector2f start;
        public readonly Vector2f end;
        private Line(float a, float b, float c,Vector2f start,Vector2f end) {
            this.a = a;
            this.b = b;
            this.c = c;
            this.start = start;
            this.end = end;
        }

        public static Line FromPoints(Vector2f pos, Vector2f pos1) {
            var a = pos.X;
            var b = pos.Y;
            var c = pos1.X;
            var d = pos1.Y;


            return new Line(d - b, a - c, a*(b-d)  - b* (a - c),pos,pos1);
        }
        



        public override string ToString() {
            return $"{a}x+{b}y+{c}=0";
        }
    }
}