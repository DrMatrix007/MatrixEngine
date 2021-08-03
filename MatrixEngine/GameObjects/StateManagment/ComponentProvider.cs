
using MatrixEngine.GameObjects.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.StateManagment {
    public class ComponentProvider<T> : Provider<T> where T : Component {
        public T data { get; set; }


        public void AddToState(T input) {
            this.data = input;
        }

        public T Get() {
            return data;
        }



        
    }
}
