using System;
using SFML.System;

namespace MatrixEngine.MatrixMath
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
            return f.IsZeroZero() ? f : f / (f.X.Sqr() + f.Y.Sqr()).Sqrt();
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

        public static int Floor(this float f)
        {
            return (int)MathF.Floor(f);
        }

        public static Vector2i Floor(this Vector2f f)
        {
            return new Vector2i(f.X.Floor(), f.Y.Floor());
        }

        public static Vector2f Multiply(this Vector2f a, Vector2f b)
        {
            return new Vector2f(a.X * b.X, a.Y * b.Y);
        }

        public static Vector2f OnlyWithX(this Vector2f v)
        {
            return new Vector2f(v.X, 0);
        }

        public static Vector2f OnlyWithY(this Vector2f v)
        {
            return new Vector2f(0, v.Y);
        }

        public static float Abs(this float f)
        {
            return MathF.Abs(f);
        }
    }
}