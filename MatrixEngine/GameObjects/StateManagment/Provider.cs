﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.StateManagment {
    public interface Provider<Output> {

        public Output data { get; set; }


        public Output Get();

    }

}
