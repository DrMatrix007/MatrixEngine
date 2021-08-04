using System;
using System.Collections.Generic;
using System.Collections;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixGDK.System.AsyncOperations {
    public class AsyncOperation {
        private readonly IEnumerator fullOperation;

        public AsyncOperation(IEnumerator fullOperation) {
            if (fullOperation == null) {
                throw new ArgumentNullException(nameof(fullOperation));
            }
            
            this.fullOperation = fullOperation;
        }
        public AsyncOperation(Func<IEnumerator> action) : this(action()) {
        }

        public bool MoveNext() {
            return fullOperation.MoveNext();
        }





    }
}
