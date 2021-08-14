using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {
    public class StateProvider<State> : Provider<State> {
        internal State data
        {
            get {
                return (this as Provider<State>).data;
            }
            set {
                (this as Provider<State>).data = value;
            }
        }
        State Provider<State>.data { get; set; }

        public StateProvider(State state) {
            data = state;
        }

        public void OperateState(Func<State,State> func) {
            data = func(data);
        }

        public State Get() {
            return data;
        }


    }

}
