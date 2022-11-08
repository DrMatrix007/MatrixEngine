#include <iostream>

#include "src/engine/matrix_engine.h"

using namespace me;

class MyApplication : public Application
{
public:
    ecs::registry reg;
};

template <int b>
class ValueComponent : public ecs::component
{
public:
    int a = 0;
    virtual ~ValueComponent(){};
};

std::unique_ptr<Application> create_main_app()
{
    using namespace ecs;
    using namespace queries;
    MyApplication *app = new MyApplication{};
    auto e = entity{};
    // auto v = me::ecs::component_vec{};
    //
    // v.set(T{}, e);
    auto &reg = app->reg;
    for (size_t i = 0; i < 10; i++)
    {
        e = entity{};
        reg.set(e, ValueComponent<0>{});
        reg.set(e, ValueComponent<1>{});
    }
    locker<int> values = locker(0);

    auto t1 = app->reg.query_async<queries::write<ValueComponent<0>>, queries::write<ValueComponent<1>>>(
        std::function([&values](guard<ValueComponent<0> *> p, guard<ValueComponent<1> *> p1)
                      {
            auto v = values.write();
            p->a = *v;
            p1->a = (*v)*(*v);
            (*v)++; 
            auto cout = me::meout.get();
            **cout << "set1: " << p->a << " " << p1->a << '\n'; }));
    auto t2 = app->reg.query_async<queries::write<ValueComponent<0>>, queries::write<ValueComponent<1>>>(
        std::function([&values](guard<ValueComponent<0> *> p, guard<ValueComponent<1> *> p1)
                      {
            auto v = values.write();
            p->a = *v;
            p1->a = (*v)*(*v);
            (*v)++; 
            
            auto cout = me::meout.get();
            **cout << "set2: " << p->a << " " << p1->a << '\n'; }));

    // auto t1 = app->reg.read_component<ValueComponent>([](const entity &e,
    // const ValueComponent *p)
    //                                         {if (!(p->a %1000)){
    //                                         auto [g,cout] = me::meout.get();
    //                                         cout << "yoo: " << p->a <<
    //                                         std::endl; } });

    // auto t2 = app->reg.read_component<ValueComponent>([](const entity &e,
    // const ValueComponent *p)
    //                                         { if (!(p->a %1000)){
    //                                         auto [g,cout] = me::meout.get();

    //                                         cout << "nice: " << p->a <<
    //                                         std::endl; } });

    // t2.join();
    // t1.join();
    // for(auto& t : wt1) {
    //     t.join();
    // }

    auto t3 = app->reg.query_sync<queries::read<ValueComponent<0>>, queries::read<ValueComponent<1>>>(
        [](guard<const ValueComponent<0> *> p, guard<const ValueComponent<1> *> p1)
        {
        auto cout = me::meout.get();
        **cout << "print1: " << p->a << "  " <<p1->a << std::endl; });
    auto t4 = app->reg.query_sync<queries::read<ValueComponent<0>>, queries::read<ValueComponent<1>>>(
        [](guard<const ValueComponent<0> *> p, guard<const ValueComponent<1> *> p1)
        {
        auto cout = me::meout.get();
        **cout << "print2: " << p->a << "  " <<p1->a << std::endl; });

    t1.join();
    t2.join();
    t3.join();
    t4.join();
    std::cout << "done!"
              << "\n";
    return std::unique_ptr<MyApplication>(app);
}