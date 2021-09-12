using MatrixEngine.Framework.Operations;
using SFML.System;
using System;
using System.Collections;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;

namespace MatrixEngine.Utilities {

    public static class Utils {

        public enum MessageType {
            Log,
            Error,
            Warning,
        }

        public static float GetTimeInSeconds(Action action) {
            var watch = new Stopwatch();

            watch.Start();

            action();

            watch.Stop();

            return (float)watch.Elapsed.TotalSeconds;
        }

        public static void LogTimeInSeconds(this Action action) {
            var s = GetTimeInSeconds(action);
            $"Time to execute: {s}".Log();
        }

        public static void Log(object message, MessageType type) {
            //var t = new Thread(new ThreadStart(() => {
            if (type == MessageType.Error) {
                Console.WriteLine($"Error: " + message);
                throw new Exception(message.ToString());
            } else if (type == MessageType.Warning) {
                Console.WriteLine($"Warning: " + message);
            } else if (type == MessageType.Log) {
                Console.WriteLine($"Log: " + message);
            }
            //}));

            //t.Start();
        }

        public static T Log<T>(this T message) {
            Log(message, MessageType.Log);
            return message;
        }

        public static void LogError(string message) {
            Log(message, MessageType.Error);
        }

        public static Vector2f OnlyWithX(this Vector2f v) {
            return new Vector2f(v.X, 0);
        }

        public static Vector2f OnlyWithY(this Vector2f v) {
            return new Vector2f(0, v.Y);
        }

        public static Random ToRandom(this Seed s) {
            return new Random(s.seed);
        }

        public static Operation ToOperation(this IEnumerator enumerator) {
            return new Operation(enumerator);
        }
    }
}