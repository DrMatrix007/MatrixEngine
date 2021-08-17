using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {
    public class StateProvider<State> : Provider<State> {
        private new State data;


        public StateProvider(State state) {
            data = state;
        }

        public void OperateState(Func<State,State> func) {
            data = func(data);
        }


        public override State Get() {
            return data;
        }


    }

}
