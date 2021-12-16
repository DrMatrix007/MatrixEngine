using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.MatrixMath;
using SFML.Graphics;
using SFML.System;

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
            return new RectangleShape() { Position = rect.Position, Size = rect.Size };
        }
        public static Vector2f ToVector2f(this float f)
        {
            return new Vector2f(f, f);
        }

    }
}