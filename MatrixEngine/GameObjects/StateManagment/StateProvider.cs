using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixGDK.GameObjects.StateManagment {
    public class StateProvider<State> : Provider<State> {
        public State data { get; set; }

        public StateProvider(State state) { 
        data = state;
        }

        public State Get() {
            return data;
        }

        
    }

}
