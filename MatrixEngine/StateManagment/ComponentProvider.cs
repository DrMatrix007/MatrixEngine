using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.GameObjects.Components;

namespace MatrixEngine.StateManagment {
    public sealed class ComponentProvider<T> : Provider<T> where T : Component {
        internal T data {
            get {
                return (this as Provider<T>).data;
            }
            set {
                (this as Provider<T>). data = value;
            }
        }
       T Provider<T>.data { get; set; }
        public void AddToState(T input) {
            data = input;
        }

        public T Get() {
            return data;
        }
    }
}
