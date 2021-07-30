
using MatrixEngine.GameObjects.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.StateManagment {
    public class ComponentProvider<T> : Provider<T> where T : Component {
        T Provider<T>.data { get; set; }


        public void AddToState(T input) {
            ((Provider<T>)this).data = input;
        }

        public T Get() {
            return ((Provider<T>)this).data;
        }



        T Provider<T>.Get() {
            return ((Provider<T>)this).data;
        }
    }
}
