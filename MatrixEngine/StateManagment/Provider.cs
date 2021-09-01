using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {

    public abstract class Provider<Output> {
        protected Output data;
        public abstract Output Get();
    }
}
