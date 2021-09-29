using MatrixEngine.Framework;
using SFML.System;
using System.Collections.Generic;
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

        public static bool IsColliding(this Rect rect1, Rect rect2) {
            return rect1.X < rect2.max.X &&
                   rect1.max.X > rect2.X &&
                   rect1.Y < rect2.max.Y &&
                   rect1.max.Y > rect2.Y;
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

            return new CollidingFix(a.IsColliding(b), new Vector2f(left.AbsMin(right), up.AbsMin(down)));
        }

        public static List<Vector2f> GetCollidingPosFromLineToRect(this Line line, Rect a) {
            var poss = new List<Vector2f>();

            foreach (var rline in a.ToLines()) {
                var ans = rline.GetCollidingPoint(line);
                poss.Add(ans);
            }

            return poss.OrderBy<Vector2f, float>((a) => line.start.Distance(a)).ToList();
        }
    }
}