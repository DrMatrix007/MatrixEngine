using System;

namespace MatrixEngine.System {
    public static class Debug {

        public enum MessageType {
            Log,
            Error,
            Warning,
        }

        public static void Log(object message, MessageType type) {
            if (type == MessageType.Error) {
                Console.WriteLine("Error:   " + message);
                throw new Exception(message.ToString());
            } else if (type == MessageType.Warning) {
                Console.WriteLine("Warning: " + message);
            } else if (type == MessageType.Log) {
                Console.WriteLine("Log:     " + message);
            }

        }
        public static void Log(object message) {
            Log(message, MessageType.Log);
        }
        public static void LogError(string message) {
            Log(message, MessageType.Error);
        }
    }

}
