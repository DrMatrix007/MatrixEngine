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

        public static Vector2f Floor(this Vector2f f)
        {
            return new Vector2f(f.X.Floor(), f.Y.Floor());
        }

        public static Vector2i FloorToInt(this Vector2f f)
        {
            return new Vector2i(f.X.Floor(), f.Y.Floor());
        }

        public static float Ceil(this float f)
        {
            return MathF.Ceiling(f);
        }

        public static Vector2f Multiply(this Vector2f a, Vector2f b)
        {
            return new Vector2f(a.X * b.X, a.Y * b.Y);
        }
        public static Vector2f Multiply(this Vector2f a, Vector2u b)
        {
            return new Vector2f(a.X * b.X, a.Y * b.Y);
        }
        public static Vector2f Multiply(this Vector2f a, Vector2i b)
        {
            return new Vector2f(a.X * b.X, a.Y * b.Y);
        }
        public static Vector2f Devide(this Vector2f a, Vector2f b)
        {
            return new Vector2f(a.X / b.X, a.Y / b.Y);
        }
        public static Vector2f Devide(this Vector2f a, Vector2u b)
        {
            return new Vector2f(a.X / b.X, a.Y / b.Y);
        }
        public static Vector2f Devide(this Vector2f a, Vector2i b)
        {
            return new Vector2f(a.X / b.X, a.Y / b.Y);
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

        public static bool IsInRange(this float f, float f1, float f2)
        {
            if (f1 > f2)
            {
                return f.IsInRange(f2, f1);
            }

            return f1 < f && f < f2;
        }

        public static bool IsInRange(this int f, int f1, int f2)
        {
            if (f1 > f2)
            {
                return f.IsInRange(f2, f1);
            }

            return f1 < f && f < f2;
        }

        public static bool IsInRangeIncludes(this float f, float f1, float f2)
        {
            if (f1 >= f2)
            {
                return f.IsInRange(f2, f1);
            }

            return f1 <= f && f <= f2;
        }

        public static bool IsInRangeIncludes(this int f, int f1, int f2)
        {
            if (f1 >= f2)
            {
                return f.IsInRange(f2, f1);
            }

            return f1 <= f && f <= f2;
        }

        public static float Min(this float a, float b)
        {
            return a < b ? a : b;
        }

        public static float Max(this float a, float b)
        {
            return a > b ? a : b;
        }


        public static Rect BigRectArea(this Rect a, Rect b)
        {
            var left = a.X.Min(b.X);
            var right = a.max.X.Max(b.max.X);
            var up = a.Y.Min(b.Y);
            var down = a.max.Y.Max(b.max.Y);


            return new Rect(left, up, right - left, down - up);
        }

        public static int Sign(this float f)
        {
            if(f < 0)
            {
                return -1;
            }
            if (f > 1)
            {
                return 1;
            }
            return 0;
        }
    }
}