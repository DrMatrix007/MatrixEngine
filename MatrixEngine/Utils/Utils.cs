using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.MatrixMath;
using SFML.Graphics;

namespace MatrixEngine.Utils
{
    public static class Utils
    {
        public static Rect ToRect(this View v)
        {
            return new Rect(v.Center - v.Size, v.Size * 2);
        }

        public static RectangleShape ToDrawableRect(this Rect rect)
        {
            return new RectangleShape() { Position = rect.position, Size = rect.size };
        }

    }
}