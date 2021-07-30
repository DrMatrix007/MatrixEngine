using MatrixEngine.GameObjects.StateManagment;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngineTests {
    class CounterProvider : Provider<int> {
        public int data { get; set; } = 0;

        public int Get() {
            return data;
        }
    }
}
