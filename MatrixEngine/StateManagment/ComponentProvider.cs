using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.GameObjects.Components;

namespace MatrixEngine.StateManagment {
    public sealed class ComponentProvider<T> : Provider<T> where T : Component {
        private new T data { get; set; }
        public void SetState(T input) {
            data = input;
        }

        public override T Get() {
            return data;
        }
    }
}
