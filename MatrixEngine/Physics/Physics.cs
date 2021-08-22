using MatrixEngine.System;
using SFML.Graphics.Glsl;
using SFML.System;
using System;
using System.Collections;
using System.Linq;

namespace MatrixEngine.Physics {
    public static class Physics {
        public struct CollidingFix {
            public bool isCollide;

            public Vector2f fixValue;


            public CollidingFix(bool isCollide, Vector2f fixValue) {
                this.fixValue = fixValue;
                this.isCollide = isCollide;
            }
        }

        public static bool isColliding(this Rect rect1, Rect rect2) {
            //float d1x = b.X - a.X - a.width;
            //float d1y = b.Y - a.Y - a.height;
            //float d2x = a.X - b.X - b.width;
            //float d2y = a.Y - b.Y - b.height;

            //var threshhold = 0.0f;

            //if ((d1x > threshhold || d1y > threshhold) || (d2x > threshhold || d2y > threshhold)) {
            //    return false;
            //}
            //return true;

            return rect1.X < rect2.X + rect2.width &&
                   rect1.X + rect1.width > rect2.X &&
                   rect1.Y < rect2.Y + rect2.height &&
                   rect1.Y + rect1.height > rect2.Y;
        }

        public static CollidingFix GetCollidingFixFromRect(this Rect a, Rect b) {
            var left = a.max.X - b.X;
            var right = a.X - b.max.X;
            var up = a.Y - b.max.Y;
            var down = a.max.Y - b.Y;

            //left = (float)Math.Round(left, 3, MidpointRounding.ToZero);
            //right = (float)Math.Round(right, 3, MidpointRounding.ToZero);
            //up = (float)Math.Round(up, 3, MidpointRounding.ToZero);
            //down = (float)Math.Round(down, 3, MidpointRounding.ToZero);

            return new CollidingFix(a.isColliding(b), new Vector2f(left.AbsMin(right), up.AbsMin(down)));
        }


        public static CollidingFix GetCollidingFixFromCircleToRect(this Circle circle, Rect rect) {
            var is_col = false;

            var is_left = circle.X > rect.cX;
            var is_up = circle.Y > rect.cY;

            var x_fix = 0.0f;
            var y_fix = 0.0f;

            if (is_left) {
                var cx = circle.X + circle.r;
                var rx = rect.X;
                x_fix = cx - rx;
                if (x_fix > 0) {
                    is_col = true;
                }
            }
            else {
                var cx = circle.X - circle.r;
                var rx = rect.max.X;
                x_fix = rx - cx;
                if (x_fix > 0) {
                    is_col = true;
                }
            }


            return new CollidingFix(is_col, new Vector2f(x_fix, y_fix));
        }
    }
}