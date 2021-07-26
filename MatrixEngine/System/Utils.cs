using System;
using System.Diagnostics;

namespace MatrixEngine.System {
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
        public static void LogTimeInSeconds(Action action) { 
            var s = GetTimeInSeconds(action);
            Utils.Log($"Time to execute: {s}");        
        }

        public static void Log(object message, MessageType type) {
            if (type == MessageType.Error) {
                Console.WriteLine($"Error: " + message);
                throw new Exception(message.ToString());
            } else if (type == MessageType.Warning) {
                Console.WriteLine($"Warning: " + message);
            } else if (type == MessageType.Log) {
                Console.WriteLine($"Log: " + message);
            }

        }
        public static void Log(this object message) {
            Log(message, MessageType.Log);
        }
        public static void LogError(string message) {
            Log(message, MessageType.Error);
        }

    }

}
