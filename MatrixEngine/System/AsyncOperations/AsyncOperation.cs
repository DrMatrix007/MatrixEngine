using System;
using System.Collections.Generic;
using System.Collections;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.System.AsyncOperations {
    public class AsyncOperation {
        private readonly IEnumerator fullOperation;

        public AsyncOperation(IEnumerator fullOperation) {
            if (fullOperation == null) {
                throw new ArgumentNullException(nameof(fullOperation));
            }
            
            this.fullOperation = fullOperation;
        }

        public bool MoveNext() {
            return fullOperation.MoveNext();
        }





    }
}
