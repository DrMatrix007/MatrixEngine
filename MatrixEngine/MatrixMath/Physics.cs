using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.MatrixMath
{
    public static class Physics
    {
        public static bool IsColliding(this Rect rect1, Rect rect2)
        {
            return rect1.X < rect2.max.X &&
                   rect1.max.X > rect2.X &&
                   rect1.Y < rect2.max.Y &&
                   rect1.max.Y > rect2.Y;
        }
    }
}