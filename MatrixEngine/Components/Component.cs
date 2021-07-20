using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Components {
    abstract class Component : Interactible {
        public bool didStart {
            get;
            private set;

        } = false;
        
        

        public abstract void OnStart();

        public abstract void OnUpdate();
    }
}
