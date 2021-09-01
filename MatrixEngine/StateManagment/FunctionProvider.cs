using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {
        public class FunctionProvider<Output> : Provider<Output> {
        private Func<Output> func;

        public FunctionProvider(Func<Output> func = null) {
            this.func = func;
        }

        public void SetFunc(Func<Output> func) {
            this.func = func;
        }

        public override Output Get() {
            if (func == null) {
                throw new NullReferenceException($"Function of provider {typeof(Output).ToString()} is null, you need to set it!");
            }
            return func();
        }
    }
}
