using System;
using System.Collections.Generic;
using System.Collections;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Framework.Operations {
    public class Operation {
        private readonly IEnumerator fullOperation;

        public Operation(IEnumerator fullOperation) {
            if (fullOperation == null) {
                throw new ArgumentNullException(nameof(fullOperation));
            }
            
            this.fullOperation = fullOperation;
        }
        public Operation(Func<IEnumerator> action) : this(action()) {
        }

        public bool MoveNext() {
            return fullOperation.MoveNext();
        }





    }
}
