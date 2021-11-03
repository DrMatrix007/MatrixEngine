using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;

namespace MatrixEngine
{
    public static class Logging
    {
        public static string GetStack()
        {
            return Environment.StackTrace.Split("\n")[2].Split("\\").Last();
        }

        private static string GetLogStack()
        {
            return Environment.StackTrace.Split("\n")[3].Split("\\").Last();
        }

        public static void Log(this object t)
        {
            var frame = new StackTrace(true).GetFrame(1);
            //Task.Run(
            //() =>
            Console.WriteLine($"{frame?.GetFileName()?.Split("\\").Last()} {frame?.GetFileLineNumber()}: {t.ToString()}")
            ;
            //);
        }
    }
}