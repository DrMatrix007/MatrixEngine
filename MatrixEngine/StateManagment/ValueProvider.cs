using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {

    public class ValueProvider<T> : Provider<T> where T : class {
        protected T value;

        public ValueProvider(T value) {
            this.value = value;
        }

        public override T Get() {
            return value;
        }
    }
}