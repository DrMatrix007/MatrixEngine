using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Framework.Operations {
    public class OperationManager {

        private App app;

        private List<Operation> asyncOperations;

        public OperationManager(App app) {
            this.app = app;
            asyncOperations = new List<Operation>();
        }
        public void AddAsyncOperation(Operation asyncOperation) {
            
            asyncOperations.Add(asyncOperation);

        }

        public void Update() {

            var list = asyncOperations.ToList();

            foreach (var item in list) {
                if (!item.MoveNext()) {
                    asyncOperations.Remove(item);
                }
            }
        }
    }
}
