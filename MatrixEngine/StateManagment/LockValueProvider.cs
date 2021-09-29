using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {
    public class LockValueProvider<T> : ValueProvider<T> where T:class {

        public bool IsLocked
        {
            get;
            private set;
        } = false;
        
        public LockValueProvider(T value) : base(value) {
        
        }
        public LockValueProvider(): base(null) {

        }

        public void Lock() {
            IsLocked = true;
        }

        public void SetValue(T value) {
            if (IsLocked) {
                throw new InvalidOperationException($"This provider<{typeof(T).Name}> is Locked!");
            }
            this.value = value;

        }

    }
}
