using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine {
    public static class Debug {

        public enum MessageType {
            Log,
            Error,
            Warning,
        }

        public static void Log(string message,MessageType type) {
            if (type == MessageType.Error) {
                Console.WriteLine("Error:   " + message);
                throw new Exception(message);
            } else if (type == MessageType.Warning) {
                Console.WriteLine("Warning: " + message);
            } else if (type == MessageType.Log) {
                Console.WriteLine("Log:     " + message);
            }

        }
        public static void Log(string message) {
            Log(message,MessageType.Log);
        }
        public static void LogError(string message) {
            Log(message,MessageType.Error);
        }
    }

}
