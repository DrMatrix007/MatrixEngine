using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.GameObjects.Components;

namespace MatrixEngine.StateManagment {

    public sealed class ComponentProvider<T> : Provider<T> where T : Component {
        private T Data { get; set; }

        public void SetState(T input) {
            Data = input;
        }

        public override T Get() {
            return Data;
        }
    }
}