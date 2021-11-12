using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.System;

namespace MatrixEngine.MatrixMath
{
    public struct Line
    {
        public readonly float a;
        public readonly float b;
        public readonly float c;

        public readonly Vector2f start;
        public readonly Vector2f end;

        private Line(float a, float b, float c, Vector2f start, Vector2f end)
        {
            this.a = a;
            this.b = b;
            this.c = c;
            this.start = start;
            this.end = end;
        }

        public Line(Vector2f pos, Vector2f pos1)
        {
            //pos.X *= -1;
            //pos1.X *= -1;

            var a = pos.X;
            var b = pos.Y;
            var c = pos1.X;
            var d = pos1.Y;

            this.a = d - b;
            this.b = a - c;
            this.c = a * (b - d) - b * (a - c);

            this.start = pos;
            this.end = pos1;
        }

        public Vector2f WhereX(float x)
        {
            return new Vector2f(x, -(a * x + c) / b);
        }

        public Vector2f WhereY(float y)
        {
            return new Vector2f(-(b * y + c) / a, y);
        }

        //public static Line FromPoints(Vector2f pos, Vector2f pos1) {
        //    var a = pos.X;
        //    var b = pos.Y;
        //    var c = pos1.X;
        //    var d = pos1.Y;

        //    return new Line(d - b, a - c, a * (b - d) - b * (a - c), pos, pos1);
        //}

        public override string ToString()
        {
            return $"{a}x+{b}y+{c}=0";
        }
    }
}