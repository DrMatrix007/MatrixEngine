using SFML.System;
using System;

namespace MatrixEngine.Framework {

    public static class MathUtils {
        public const float TOLERANCE = 0.01f;

        public static float Sqrt(this float x) {
            return (float)MathF.Sqrt(x);
        }

        public static float Pow(this float x, float pow) {
            return (float)MathF.Pow(x, pow);
        }

        public static float Sqr(this float x) {
            return x.Pow(2);
        }

        public static Vector2f Normalize(this Vector2f v) {
            if (v.X == 0 && v.Y == 0) {
                return v;
            }

            var a = (v.X.Sqr() + v.Y.Sqr()).Sqrt();
            return v / a;
        }

        public static Vector2f LerpToZero(this Vector2f v, float x) {
            return v.Lerp(new Vector2f(0, 0), x);
        }

        public static Vector2f Lerp(this Vector2f v1, Vector2f v2, float t) {
            return new Vector2f(Lerp(v1.X, v2.X, t), Lerp(v1.Y, v2.Y, t));
        }

        public static float Lerp(float a, float b, float t) {
            return (1 - t) * a + t * b;
        }

        public static float LerpToZero(float a, float t) {
            return Lerp(a, 0, t);
        }

        public static float Length(this Vector2f v) {
            return (v.X.Sqr() + v.Y.Sqr()).Sqrt();
        }

        public static float Distance(this Vector2f v1, Vector2f v2) {
            return (v1 - v2).Length();
        }

        public static Vector2f Round(this Vector2f v, int r) {
            return new Vector2f((float)Math.Round(v.X, r, MidpointRounding.ToZero),
                MathF.Round(v.Y, r, MidpointRounding.ToZero));
        }

        public static Vector2f Round(this Vector2f v, MidpointRounding r) {
            return new Vector2f((float)Math.Round(v.X, 0, r),
                MathF.Round(v.Y, 0, r));
        }

        public static Vector2f Multiply(this Vector2f v1, Vector2f v2) {
            return new Vector2f(v1.X * v2.X, v1.Y * v2.Y);
        }

        public static Vector2f Devide(this Vector2f v1, Vector2f v2) {
            return new Vector2f(v1.X / v2.X, v1.Y / v2.Y);
        }

        public static Vector2i Multiply(this Vector2i v1, Vector2i v2) {
            return new Vector2i(v1.X * v2.X, v1.Y * v2.Y);
        }

        public static Vector2i Devide(this Vector2i v1, Vector2i v2) {
            return new Vector2i(v1.X / v2.X, v1.Y / v2.Y);
        }

        public static float Abs(this float f) {
            return MathF.Abs(f);
        }

        public static float Round(this float f, int r) {
            return MathF.Round(f, r);
        }

        public static float AbsMin(this float f, float r) {
            return (MathF.Min(r.Abs(), f.Abs()) - r.Abs()).Abs() < 0.001f ? r : f;
        }

        public static bool IsBetween(this float f, float small, float big) {
            return small <= f && f <= big;
        }

        public static float Average(this Vector2f v) {
            return (v.X + v.Y) / 2;
        }

        public static Vector2f Min(this Vector2f v) {
            var a = Math.Min(v.X, v.Y);
            return new Vector2f(a, a);
        }

        public static bool BiggerThan(this Vector2f v, Vector2f v1) {
            return v.X > v1.X || v.Y > v1.Y;
        }

        public static bool SmallerThan(this Vector2f v, Vector2f v1) {
            return !v.BiggerThan(v1);
        }

        public static Vector2f GetCollidingPoint(this Line l1, Line l2) {
            try {
                var A = l1.a;
                var B = l1.b;
                var C = l1.c;
                var a = l2.a;
                var b = l2.b;
                var d = l2.c;

                var y = (d * A - C * a) / (B * a - b * A);

                var x = (B * d - C * b) / (A * b - B * a);

                if (float.IsInfinity(y) || float.IsInfinity(x)) {
                    return new Vector2f(float.PositiveInfinity, float.PositiveInfinity);
                }

                var pos = new Vector2f(x, y);

                if (l1.IsOnRange(pos) && l2.IsOnRange(pos) && float.IsFinite(pos.X) && float.IsFinite(pos.Y)) {
                    return pos;
                }

                return new Vector2f(float.PositiveInfinity, float.PositiveInfinity);
            } catch (DivideByZeroException) {
                return new Vector2f(float.PositiveInfinity, float.PositiveInfinity);
            }
        }

        public static bool IsOnLine(this Line line, Vector2f pos) {
            Console.WriteLine((line.a * pos.X + line.b * pos.Y + line.c).Abs() < TOLERANCE);
            return (line.a * pos.X + line.b * pos.Y + line.c).Abs() < TOLERANCE
                   && line.IsOnRange(pos);
        }

        public static bool IsOnRange(this Line line, Vector2f pos) {
            return pos.X.IsBetween(MathF.Min(line.start.X, line.end.X), MathF.Max(line.start.X, line.end.X)) &&
                   pos.Y.IsBetween(MathF.Min(line.start.Y, line.end.Y), MathF.Max(line.start.Y, line.end.Y));
            ;
        }

        public static int Sign(this float f) {
            if (f > 0) {
                return 1;
            }
            if (f < 0) {
                return -1;
            }
            return 0;
        }

        public static Vector2f Abs(this Vector2f f) {
            return new Vector2f(MathF.Abs(f.X), MathF.Abs(f.Y));
        }

        public static Vector2i Abs(this Vector2i f) {
            return new Vector2i(Math.Abs(f.X), Math.Abs(f.Y));
        }

        public static float Floor(this float f) {
            return MathF.Floor(f);
        }
    }
}