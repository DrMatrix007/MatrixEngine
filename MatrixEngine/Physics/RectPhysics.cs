using System.Linq;
namespace MatrixEngine.Physics {
    public static partial class Physics {


        public enum CollidingAxis {
            None,
            X,
            Y,
        }
        public struct CollidingFix {
            public CollidingAxis axis;
            public float fixValue;

            public static readonly CollidingFix None = new CollidingFix(CollidingAxis.None, 0);

            public CollidingFix(CollidingAxis axis, float fixValue) {
                this.axis = axis;
                this.fixValue = fixValue;
            }

        }
        public static bool isColliding(this Rect a, Rect b) {
            float d1x = b.X - a.X - a.width;
            float d1y = b.Y - a.Y - a.height;
            float d2x = a.X - b.X - b.width;
            float d2y = a.Y - b.Y - b.height;

            if (d1x > 0 || d1y > 0) {
                return false;
            }
            if (d2x > 0 || d2y > 0) {

                return false;
            }
            return true;
        }
        public static CollidingFix GetCollidingFixFromB(this Rect a, Rect b) {

            if (!a.isColliding(b)) {
                return CollidingFix.None;
            }

            var left = a.Max.X - b.X;
            var right = a.X - b.Max.X;
            var up = a.Y - b.Max.Y;
            var down = a.Max.Y - b.Y;

            //left = (float)Math.Round(left, 3, MidpointRounding.ToZero);
            //right = (float)Math.Round(right, 3, MidpointRounding.ToZero);
            //up = (float)Math.Round(up, 3, MidpointRounding.ToZero);
            //down = (float)Math.Round(down, 3, MidpointRounding.ToZero);


            float[] f = new float[] { left, right, up, down };

            var val = f.Aggregate((a, b) => { return System.Math.Abs(a) > System.Math.Abs(b) ? b : a; });

            var index = f.ToList().IndexOf(val);




            if (index <= 1) {
                return new CollidingFix(CollidingAxis.X, index == 0 ? left : right);
            } else {
                return new CollidingFix(CollidingAxis.Y, index == 2 ? up : down);
            }

            //Debug.Log($"{left.ToString("0.0")}, {right.ToString("0.0")}, {up.ToString("0.0")}, {down.ToString("0.0")}");


        }

    }
}
