using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.System;

namespace MatrixEngine
{
    internal static class MatrixMath
    {
        public static float Sqrt(this float f)
        {
            return MathF.Sqrt(f);
        }

        public static float Sqr(this float f)
        {
            return f * f;
        }

        public static Vector2f Normalized(this Vector2f f)
        {
            return f / (f.X.Sqr() + f.Y.Sqr()).Sqrt();
        }

        public static float Length(this Vector2f f)
        {
            return (f.X.Sqr() + f.Y.Sqr()).Sqrt();
        }

        public static bool IsZeroZero(this Vector2f f)
        {
            return f.X == 0 && f.Y == 0;
        }

        public static bool IsFinite(this float f)
        {
            return float.IsFinite(f);
        }

        public static bool IsFinite(this Vector2f f)
        {
            return f.X.IsFinite() && f.Y.IsFinite();
        }

        public static float Pow(this float f, float p)
        {
            return MathF.Pow(f, p);
        }

        public static float Pow(this int i, float p)
        {
            return MathF.Pow(i, p);
        }
    }
}