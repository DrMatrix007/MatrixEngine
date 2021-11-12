using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using SFML.System;

namespace MatrixEngine.Utils
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

        public static T Log<T>(this T t)
        {
            Console.WriteLine($"Info: {t.ToString()}");
            return t;
        }

        public static T TaskLog<T>(this T t)
        {
            Task.Run(
                () =>
                {
                    Console.WriteLine($"Info: {t.ToString()}");
                });
            return t;
        }
        public static T LogTime<T>(this Func<T> action)
        {
            var c = Stopwatch.StartNew();
            var a = action();
            c.Stop();
            
            $"Time of: {action.Method.Name}: {c.Elapsed.TotalSeconds}".Log();
            
            return a;
        }

        public static void Assert(bool v)
        {
            if (!v)
            {
                throw new Exception($"Failed Assert");
            }
        }

        public static void Assert(bool v,string message)
        {
            if (!v)
            {
                throw new Exception($"Failed Assert - message: {message}");
            }
        }

        public static void LogTime(this Action action)
        {
            var c = Stopwatch.StartNew();
            action();
            c.Stop();
            $"Time of: {action.Method.Name}: {c.Elapsed.TotalSeconds}".Log();
        }
    }
}