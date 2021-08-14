using SFML.System;
using System;
using System.Diagnostics.CodeAnalysis;

namespace MatrixEngine.System {
    public static class MathUtils {
        public static float Sqrt(this float x) {
            return (float) MathF.Sqrt(x);
        }

        public static float Pow(this float x, float pow) {
            return (float) MathF.Pow(x, pow);
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
            return new Vector2f((float) Math.Round(v.X, r, MidpointRounding.ToZero),
                (float) Math.Round(v.Y, r, MidpointRounding.ToZero));
        }

        public static Vector2f Round(this Vector2f v, MidpointRounding r) {
            return new Vector2f((float) Math.Round(v.X, 0, r), (float) Math.Round(v.Y, 0, r));
        }

        public static Vector2f Multiply(this Vector2f v1, Vector2f v2) {
            return new Vector2f(v1.X * v2.X, v1.Y * v2.Y);
        }

        public static Vector2f Devide(this Vector2f v1, Vector2f v2) {
            return new Vector2f(v1.X / v2.X, v1.Y / v2.Y);
        }

        public static float Abs(this float f) {
            return MathF.Abs(f);
        }

        public static float Round(this float f, int r) {
            return MathF.Round(f, r);
        }

        public static float AbsMin(this float f, float r) {
            if (Math.Abs(MathF.Min(r.Abs(), f.Abs()) - r.Abs()) < 0.001f) {
                return r;
            }

            return f;
        }

        public static bool IsBetween(this float f, float small, float big) {
            return small < f && f < big;
        }

        public static float Average(this Vector2f v) {
            return (v.X + v.Y) / 2;
        }

        public static Vector2f Min(this Vector2f v) {
            var a = Math.Min(v.X, v.Y);
            return new Vector2f(a,a);
        }

        public static bool BiggerThan(this Vector2f v, Vector2f v1) {
            return v.X > v1.X || v.Y > v1.Y;
        }
        public static bool SmallerThan(this Vector2f v, Vector2f v1) {
            return !v.BiggerThan(v1);
        }

    }
}