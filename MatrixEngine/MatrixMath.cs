using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

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
    }
}